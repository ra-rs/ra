#[doc = "Register `LVCMPCR` reader"]
pub struct R(crate::R<LVCMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVCMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVCMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVCMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVCMPCR` writer"]
pub struct W(crate::W<LVCMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVCMPCR_SPEC>;
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
impl From<crate::W<LVCMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVCMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVD1E` reader - Voltage Detection 1 Enable"]
pub type LVD1E_R = crate::BitReader<LVD1E_A>;
#[doc = "Voltage Detection 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1E_A {
    #[doc = "0: Voltage detection 1 circuit disabled"]
    _0 = 0,
    #[doc = "1: Voltage detection 1 circuit enabled"]
    _1 = 1,
}
impl From<LVD1E_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1E_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD1E_A {
        match self.bits {
            false => LVD1E_A::_0,
            true => LVD1E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1E_A::_1
    }
}
#[doc = "Field `LVD1E` writer - Voltage Detection 1 Enable"]
pub type LVD1E_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVCMPCR_SPEC, LVD1E_A, O>;
impl<'a, const O: u8> LVD1E_W<'a, O> {
    #[doc = "Voltage detection 1 circuit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD1E_A::_0)
    }
    #[doc = "Voltage detection 1 circuit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD1E_A::_1)
    }
}
#[doc = "Field `LVD2E` reader - Voltage Detection 2 Enable"]
pub type LVD2E_R = crate::BitReader<LVD2E_A>;
#[doc = "Voltage Detection 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2E_A {
    #[doc = "0: Voltage detection 2 circuit disabled"]
    _0 = 0,
    #[doc = "1: Voltage detection 2 circuit enabled"]
    _1 = 1,
}
impl From<LVD2E_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2E_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD2E_A {
        match self.bits {
            false => LVD2E_A::_0,
            true => LVD2E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2E_A::_1
    }
}
#[doc = "Field `LVD2E` writer - Voltage Detection 2 Enable"]
pub type LVD2E_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVCMPCR_SPEC, LVD2E_A, O>;
impl<'a, const O: u8> LVD2E_W<'a, O> {
    #[doc = "Voltage detection 2 circuit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD2E_A::_0)
    }
    #[doc = "Voltage detection 2 circuit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD2E_A::_1)
    }
}
impl R {
    #[doc = "Bit 5 - Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(&self) -> LVD1E_R {
        LVD1E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(&self) -> LVD2E_R {
        LVD2E_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Voltage Detection 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1e(&mut self) -> LVD1E_W<5> {
        LVD1E_W::new(self)
    }
    #[doc = "Bit 6 - Voltage Detection 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2e(&mut self) -> LVD2E_W<6> {
        LVD2E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Monitor Circuit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvcmpcr](index.html) module"]
pub struct LVCMPCR_SPEC;
impl crate::RegisterSpec for LVCMPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvcmpcr::R](R) reader structure"]
impl crate::Readable for LVCMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvcmpcr::W](W) writer structure"]
impl crate::Writable for LVCMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVCMPCR to value 0"]
impl crate::Resettable for LVCMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
