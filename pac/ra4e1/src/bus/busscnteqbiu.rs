#[doc = "Register `BUSSCNTEQBIU` reader"]
pub struct R(crate::R<BUSSCNTEQBIU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSCNTEQBIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSCNTEQBIU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSCNTEQBIU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSSCNTEQBIU` writer"]
pub struct W(crate::W<BUSSCNTEQBIU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSSCNTEQBIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BUSSCNTEQBIU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSSCNTEQBIU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBS` reader - Arbitration Select for two masters"]
pub type ARBS_R = crate::FieldReader<u8, ARBS_A>;
#[doc = "Arbitration Select for two masters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBS_A {
    #[doc = "0: DMAC/DTC > CPU"]
    _00 = 0,
    #[doc = "1: DMAC/DTC ↔ CPU"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<ARBS_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBS_A) -> Self {
        variant as _
    }
}
impl ARBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBS_A {
        match self.bits {
            0 => ARBS_A::_00,
            1 => ARBS_A::_01,
            2 => ARBS_A::_10,
            3 => ARBS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ARBS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ARBS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ARBS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ARBS_A::_11
    }
}
#[doc = "Field `ARBS` writer - Arbitration Select for two masters"]
pub type ARBS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BUSSCNTEQBIU_SPEC, u8, ARBS_A, 2, O>;
impl<'a, const O: u8> ARBS_W<'a, O> {
    #[doc = "DMAC/DTC > CPU"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ARBS_A::_00)
    }
    #[doc = "DMAC/DTC ↔ CPU"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ARBS_A::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ARBS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ARBS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Arbitration Select for two masters"]
    #[inline(always)]
    pub fn arbs(&self) -> ARBS_R {
        ARBS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Arbitration Select for two masters"]
    #[inline(always)]
    #[must_use]
    pub fn arbs(&mut self) -> ARBS_W<0> {
        ARBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Bus Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busscnteqbiu](index.html) module"]
pub struct BUSSCNTEQBIU_SPEC;
impl crate::RegisterSpec for BUSSCNTEQBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [busscnteqbiu::R](R) reader structure"]
impl crate::Readable for BUSSCNTEQBIU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busscnteqbiu::W](W) writer structure"]
impl crate::Writable for BUSSCNTEQBIU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSCNTEQBIU to value 0"]
impl crate::Resettable for BUSSCNTEQBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
