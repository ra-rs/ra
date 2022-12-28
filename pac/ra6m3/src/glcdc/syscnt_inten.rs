#[doc = "Register `SYSCNT_INTEN` reader"]
pub struct R(crate::R<SYSCNT_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCNT_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCNT_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCNT_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCNT_INTEN` writer"]
pub struct W(crate::W<SYSCNT_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCNT_INTEN_SPEC>;
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
impl From<crate::W<SYSCNT_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCNT_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VPOSINTEN` reader - Interrupt request signal GLCDC_VPOS enable control."]
pub type VPOSINTEN_R = crate::BitReader<VPOSINTEN_A>;
#[doc = "Interrupt request signal GLCDC_VPOS enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPOSINTEN_A {
    #[doc = "1: Enables GLCDC_VPOS output"]
    _1 = 1,
    #[doc = "0: Disables GLCDC_VPOS output"]
    _0 = 0,
}
impl From<VPOSINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: VPOSINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VPOSINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPOSINTEN_A {
        match self.bits {
            true => VPOSINTEN_A::_1,
            false => VPOSINTEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPOSINTEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPOSINTEN_A::_0
    }
}
#[doc = "Field `VPOSINTEN` writer - Interrupt request signal GLCDC_VPOS enable control."]
pub type VPOSINTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCNT_INTEN_SPEC, VPOSINTEN_A, O>;
impl<'a, const O: u8> VPOSINTEN_W<'a, O> {
    #[doc = "Enables GLCDC_VPOS output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VPOSINTEN_A::_1)
    }
    #[doc = "Disables GLCDC_VPOS output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VPOSINTEN_A::_0)
    }
}
#[doc = "Field `L1UNDFINTEN` reader - Interrupt request signal GLCDC_L1UNDF enable control."]
pub type L1UNDFINTEN_R = crate::BitReader<L1UNDFINTEN_A>;
#[doc = "Interrupt request signal GLCDC_L1UNDF enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1UNDFINTEN_A {
    #[doc = "1: Enables GLCDC_L1UNDF output"]
    _1 = 1,
    #[doc = "0: Disables GLCDC_L1UNDF output"]
    _0 = 0,
}
impl From<L1UNDFINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: L1UNDFINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl L1UNDFINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1UNDFINTEN_A {
        match self.bits {
            true => L1UNDFINTEN_A::_1,
            false => L1UNDFINTEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1UNDFINTEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1UNDFINTEN_A::_0
    }
}
#[doc = "Field `L1UNDFINTEN` writer - Interrupt request signal GLCDC_L1UNDF enable control."]
pub type L1UNDFINTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCNT_INTEN_SPEC, L1UNDFINTEN_A, O>;
impl<'a, const O: u8> L1UNDFINTEN_W<'a, O> {
    #[doc = "Enables GLCDC_L1UNDF output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L1UNDFINTEN_A::_1)
    }
    #[doc = "Disables GLCDC_L1UNDF output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L1UNDFINTEN_A::_0)
    }
}
#[doc = "Field `L2UNDFINTEN` reader - Interrupt request signal GLCDC_L2UNDF enable control."]
pub type L2UNDFINTEN_R = crate::BitReader<L2UNDFINTEN_A>;
#[doc = "Interrupt request signal GLCDC_L2UNDF enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2UNDFINTEN_A {
    #[doc = "1: Enables GLCDC_L2UNDF output"]
    _1 = 1,
    #[doc = "0: Disables GLCDC_L2UNDF output"]
    _0 = 0,
}
impl From<L2UNDFINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: L2UNDFINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl L2UNDFINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L2UNDFINTEN_A {
        match self.bits {
            true => L2UNDFINTEN_A::_1,
            false => L2UNDFINTEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L2UNDFINTEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L2UNDFINTEN_A::_0
    }
}
#[doc = "Field `L2UNDFINTEN` writer - Interrupt request signal GLCDC_L2UNDF enable control."]
pub type L2UNDFINTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCNT_INTEN_SPEC, L2UNDFINTEN_A, O>;
impl<'a, const O: u8> L2UNDFINTEN_W<'a, O> {
    #[doc = "Enables GLCDC_L2UNDF output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(L2UNDFINTEN_A::_1)
    }
    #[doc = "Disables GLCDC_L2UNDF output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(L2UNDFINTEN_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt request signal GLCDC_VPOS enable control."]
    #[inline(always)]
    pub fn vposinten(&self) -> VPOSINTEN_R {
        VPOSINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt request signal GLCDC_L1UNDF enable control."]
    #[inline(always)]
    pub fn l1undfinten(&self) -> L1UNDFINTEN_R {
        L1UNDFINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt request signal GLCDC_L2UNDF enable control."]
    #[inline(always)]
    pub fn l2undfinten(&self) -> L2UNDFINTEN_R {
        L2UNDFINTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt request signal GLCDC_VPOS enable control."]
    #[inline(always)]
    #[must_use]
    pub fn vposinten(&mut self) -> VPOSINTEN_W<0> {
        VPOSINTEN_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt request signal GLCDC_L1UNDF enable control."]
    #[inline(always)]
    #[must_use]
    pub fn l1undfinten(&mut self) -> L1UNDFINTEN_W<1> {
        L1UNDFINTEN_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt request signal GLCDC_L2UNDF enable control."]
    #[inline(always)]
    #[must_use]
    pub fn l2undfinten(&mut self) -> L2UNDFINTEN_W<2> {
        L2UNDFINTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Block Interrupt Request Enable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscnt_inten](index.html) module"]
pub struct SYSCNT_INTEN_SPEC;
impl crate::RegisterSpec for SYSCNT_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscnt_inten::R](R) reader structure"]
impl crate::Readable for SYSCNT_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscnt_inten::W](W) writer structure"]
impl crate::Writable for SYSCNT_INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCNT_INTEN to value 0"]
impl crate::Resettable for SYSCNT_INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
