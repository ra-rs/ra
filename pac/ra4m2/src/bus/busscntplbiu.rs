#[doc = "Register `BUSSCNTPLBIU` reader"]
pub struct R(crate::R<BUSSCNTPLBIU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSCNTPLBIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSCNTPLBIU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSCNTPLBIU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSSCNTPLBIU` writer"]
pub struct W(crate::W<BUSSCNTPLBIU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSSCNTPLBIU_SPEC>;
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
impl From<crate::W<BUSSCNTPLBIU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSSCNTPLBIU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBS` reader - Arbitration Select for two masters"]
pub type ARBS_R = crate::BitReader<ARBS_A>;
#[doc = "Arbitration Select for two masters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBS_A {
    #[doc = "0: DMAC/DTC > CPU"]
    _0 = 0,
    #[doc = "1: DMAC/DTC ↔ CPU"]
    _1 = 1,
}
impl From<ARBS_A> for bool {
    #[inline(always)]
    fn from(variant: ARBS_A) -> Self {
        variant as u8 != 0
    }
}
impl ARBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBS_A {
        match self.bits {
            false => ARBS_A::_0,
            true => ARBS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARBS_A::_1
    }
}
#[doc = "Field `ARBS` writer - Arbitration Select for two masters"]
pub type ARBS_W<'a, const O: u8> = crate::BitWriter<'a, u16, BUSSCNTPLBIU_SPEC, ARBS_A, O>;
impl<'a, const O: u8> ARBS_W<'a, O> {
    #[doc = "DMAC/DTC > CPU"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARBS_A::_0)
    }
    #[doc = "DMAC/DTC ↔ CPU"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARBS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Arbitration Select for two masters"]
    #[inline(always)]
    pub fn arbs(&self) -> ARBS_R {
        ARBS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Arbitration Select for two masters"]
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
#[doc = "Slave Bus Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busscntplbiu](index.html) module"]
pub struct BUSSCNTPLBIU_SPEC;
impl crate::RegisterSpec for BUSSCNTPLBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [busscntplbiu::R](R) reader structure"]
impl crate::Readable for BUSSCNTPLBIU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busscntplbiu::W](W) writer structure"]
impl crate::Writable for BUSSCNTPLBIU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSCNTPLBIU to value 0"]
impl crate::Resettable for BUSSCNTPLBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
