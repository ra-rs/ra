#[doc = "Register `IIRCH%sINT` reader"]
pub struct R(crate::R<IIRCHINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRCHINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRCHINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRCHINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIRCH%sINT` writer"]
pub struct W(crate::W<IIRCHINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIRCHINT_SPEC>;
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
impl From<crate::W<IIRCHINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIRCHINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPRCFIE` reader - Channel processing completion interrupt enable bit"]
pub type CPRCFIE_R = crate::BitReader<CPRCFIE_A>;
#[doc = "Channel processing completion interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPRCFIE_A {
    #[doc = "0: The generation of channel processing completion interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: The generation of channel processing completion interrupt requests is enabled."]
    _1 = 1,
}
impl From<CPRCFIE_A> for bool {
    #[inline(always)]
    fn from(variant: CPRCFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CPRCFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPRCFIE_A {
        match self.bits {
            false => CPRCFIE_A::_0,
            true => CPRCFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPRCFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPRCFIE_A::_1
    }
}
#[doc = "Field `CPRCFIE` writer - Channel processing completion interrupt enable bit"]
pub type CPRCFIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IIRCHINT_SPEC, CPRCFIE_A, O>;
impl<'a, const O: u8> CPRCFIE_W<'a, O> {
    #[doc = "The generation of channel processing completion interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPRCFIE_A::_0)
    }
    #[doc = "The generation of channel processing completion interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPRCFIE_A::_1)
    }
}
#[doc = "Field `ORDYIE` reader - Output data preparation completion interrupt enable bit"]
pub type ORDYIE_R = crate::BitReader<ORDYIE_A>;
#[doc = "Output data preparation completion interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORDYIE_A {
    #[doc = "0: The generation of output data preparation completion interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: The generation of output data preparation completion interrupt requests is enabled."]
    _1 = 1,
}
impl From<ORDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: ORDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ORDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORDYIE_A {
        match self.bits {
            false => ORDYIE_A::_0,
            true => ORDYIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORDYIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORDYIE_A::_1
    }
}
#[doc = "Field `ORDYIE` writer - Output data preparation completion interrupt enable bit"]
pub type ORDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IIRCHINT_SPEC, ORDYIE_A, O>;
impl<'a, const O: u8> ORDYIE_W<'a, O> {
    #[doc = "The generation of output data preparation completion interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORDYIE_A::_0)
    }
    #[doc = "The generation of output data preparation completion interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORDYIE_A::_1)
    }
}
#[doc = "Field `CERRIE` reader - Operation error interrupt enable bit"]
pub type CERRIE_R = crate::BitReader<CERRIE_A>;
#[doc = "Operation error interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERRIE_A {
    #[doc = "0: The generation of operation error interrupt requests is disabled."]
    _0 = 0,
    #[doc = "1: The generation of operation error interrupt requests is enabled."]
    _1 = 1,
}
impl From<CERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: CERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERRIE_A {
        match self.bits {
            false => CERRIE_A::_0,
            true => CERRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CERRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CERRIE_A::_1
    }
}
#[doc = "Field `CERRIE` writer - Operation error interrupt enable bit"]
pub type CERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IIRCHINT_SPEC, CERRIE_A, O>;
impl<'a, const O: u8> CERRIE_W<'a, O> {
    #[doc = "The generation of operation error interrupt requests is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CERRIE_A::_0)
    }
    #[doc = "The generation of operation error interrupt requests is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CERRIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Channel processing completion interrupt enable bit"]
    #[inline(always)]
    pub fn cprcfie(&self) -> CPRCFIE_R {
        CPRCFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output data preparation completion interrupt enable bit"]
    #[inline(always)]
    pub fn ordyie(&self) -> ORDYIE_R {
        ORDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operation error interrupt enable bit"]
    #[inline(always)]
    pub fn cerrie(&self) -> CERRIE_R {
        CERRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Channel processing completion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cprcfie(&mut self) -> CPRCFIE_W<1> {
        CPRCFIE_W::new(self)
    }
    #[doc = "Bit 2 - Output data preparation completion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ordyie(&mut self) -> ORDYIE_W<2> {
        ORDYIE_W::new(self)
    }
    #[doc = "Bit 3 - Operation error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cerrie(&mut self) -> CERRIE_W<3> {
        CERRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirchint](index.html) module"]
pub struct IIRCHINT_SPEC;
impl crate::RegisterSpec for IIRCHINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iirchint::R](R) reader structure"]
impl crate::Readable for IIRCHINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iirchint::W](W) writer structure"]
impl crate::Writable for IIRCHINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIRCH%sINT to value 0"]
impl crate::Resettable for IIRCHINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
